//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2782/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2782<F: Float>(t1561: F, t40360: F, t14843: F, t40864: F, t10779: F, t14931: F, t1548: F, t2724: F, t10811: F, t14693: F, t40850: F, t40851: F, t40855: F, t51074: F, t51079: F, t51081: F, t51083: F, t51086: F, t51089: F, t51093: F, t51096: F, t51099: F, t51100: F, t51102: F) -> F {
    let t51104 = t40360 * t1561;
    let t51106 = t40864 * t14843;
    let t51110 = t14931 * t10779 * t1548 * t2724;
    let t51112 = t10811 * t14693;
    let t51114 = -F::cast_from(0.91464571985215438874e-3_f64) * t51074 - t51079 + F::cast_from(0.27107389498472794075e-4_f64) * t51081 + F::cast_from(0.45178982497454656792e-6_f64) * t51083 + F::cast_from(0.54214778996945588151e-4_f64) * t51086 + F::cast_from(0.5421477899694558815e-4_f64) * t51089 - t51093 - t40850 + F::cast_from(0.45732285992607719436e-2_f64) * t40851 - t51096 + F::cast_from(0.15246000842785598467e-3_f64) * t40855 - t51099 - F::cast_from(0.12846167376791569079e-2_f64) * t51100 + F::cast_from(0.91464571985215438873e-3_f64) * t51102 + F::cast_from(0.37792653007779990369e-1_f64) * t51104 + F::new(7.0) / F::new(4.0) * t51106 - F::cast_from(0.30492001685571196935e-3_f64) * t51110 - F::cast_from(0.24009450146119052704e-1_f64) * t51112;
    t51114
}
