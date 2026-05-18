//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1359/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1359<F: Float>(t26966: F, t26999: F, t27042: F, t27070: F, t27077: F, t28132: F, t28137: F, t28179: F, t28204: F, t7772: F, t7788: F, t95909: F, t95913: F, t97039: F, t97051: F, t97056: F, t97060: F, t97063: F) -> F {
    let t97066 = F::new(0.23214722222222222222e-2) * t95909 - F::new(0.185671721767578125e-4) * t27077 * t97039 - F::new(0.92754700520833333334e-4) * t28204 * t26999 - F::new(0.92754700520833333334e-4) * t27070 * t28132 + F::new(0.37069444444444444444e-2) * t26966 * t28179 - F::new(0.38691203703703703704e-2) * t95913 + t97051 + F::new(0.74203760416666666667e-3) * t27042 * t28137 - F::new(0.13913205078125e-3) * t7772 * t97056 - t97060 - F::new(0.34752604166666666667e-3) * t7788 * t97063;
    t97066
}
