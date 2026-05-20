//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2035/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2035<F: Float>(t100964: F, t100975: F, t100982: F, t101016: F, t101065: F, t101093: F, t102854: F, t102864: F, t102877: F, t102888: F, t102917: F, t103586: F, t1940: F, t2403: F, t25760: F, t25763: F, t25778: F, t26425: F, t26585: F, t27764: F, t27806: F, t27817: F, t28472: F, t7207: F, t7432: F, t8020: F) -> F {
    let t103778 = -F::new(3.0) * t102888 * t25760 + F::new(2.0) * t28472 * t101016 + t28472 * t101065 - t1940 * t7432 * t101093 / F::new(2.0) - F::new(3.0) * t28472 * t100982 + t102877 + t1940 * t103586 * t25778 - t1940 * t26585 * t27817 + F::new(6.0) * t102864 * t27764 - F::new(3.0) / F::new(2.0) * t26425 * t100964 + t102917 + F::new(2.0) * t28472 * t100975 - t1940 * t26585 * t27806 - t1940 * t102854 * t7207 + F::new(3.0) * t2403 * t8020 * t25763;
    t103778
}
