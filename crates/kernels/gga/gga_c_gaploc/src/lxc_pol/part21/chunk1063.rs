//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1063/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1063<F: Float>(t2321: F, t27071: F, t9074: F, t10555: F, t169: F, t31548: F, t6490: F, t1365: F, t25735: F, t6525: F, t10241: F, t4324: F) -> (F, F, F, F) {
    let t32020 = t9074 * t27071 * t2321;
    let t32021 = 0.11856252764865062333e-2 * t32020;
    let t32025 = 0.68292015925622759036e0 * t31548 * t10555 * t169 * t6490;
    let t32027 = t6525 * t1365 * t25735;
    let t32028 = 0.23712505529730124666e-2 * t32027;
    let t32033 = t10241 * t4324;
    (t32021, t32025, t32028, t32033)
}
