//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1962/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1962<F: Float>(t2031: F, t90090: F, t26012: F, t7031: F, t22549: F, t90094: F, t26009: F, t84219: F, t90247: F, t23963: F, t23970: F, t26016: F, t26954: F, t83722: F, t83778: F, t84183: F, t84190: F, t90076: F, t90080: F, t90114: F) -> F {
    let t92040 = t2031 * t90090;
    let t92047 = t7031 * t26012;
    let t92049 = F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t22549 * t92047;
    let t92052 = t2031 * t90094;
    let t92056 = F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t84219 * t26009;
    let t92057 = t2031 * t90247;
    let t92068 = F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t22549 * t92040 + F::cast_from(20.0_f64) * t23963 * t90076 + F::cast_from(10.0_f64) * t23963 * t90080 - t92049 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t83778 * t26954 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t22549 * t92052 - t92056 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t22549 * t92057 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t84183 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t90114 * t23970 + F::cast_from(20.0_f64) * t84190 * t26009 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t83722 * t26954;
    t92068
}
