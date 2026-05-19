//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 910/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk910<F: Float>(t11849: F, t2628: F, t43646: F, t43652: F, t43657: F, t10914: F, t10915: F, t1445: F, t2684: F, t41060: F, t41071: F, t43609: F, t43611: F, t43650: F, t44940: F, t45350: F, t45415: F, t45421: F, t45426: F, t45429: F, t45432: F, t45438: F, t45440: F, t7427: F, t7573: F, t7585: F, t833: F) -> F {
    let t45441 = t11849 * t2628;
    let t45442 = F::cast_from(0.29792074959875355558e-1_f64) * t45441;
    let t45451 = F::cast_from(0.17875244975925213335e0_f64) * t43646;
    let t45453 = F::cast_from(0.30674340763136599741e1_f64) * t43652;
    let t45454 = F::cast_from(0.20449560508757733161e1_f64) * t43657;
    let t45455 = t45415 + F::cast_from(0.2556195063594716645e0_f64) * t41060 - F::cast_from(0.2556195063594716645e0_f64) * t41071 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t1445 * t44940 - F::cast_from(0.44688112439813033338e-1_f64) * t45421 + t45426 - t45429 - t45432 - F::cast_from(0.42900587942220512004e1_f64) * t10914 * t10915 * t45350 - t45438 + t45440 - t45442 + F::cast_from(0.87421871174939309263e2_f64) * t2684 * t7585 * t45350 - F::cast_from(0.12423108009070322895e3_f64) * t7427 * t7573 * t45350 + F::cast_from(0.76685851907841499353e0_f64) * t43609 + F::cast_from(0.76685851907841499353e0_f64) * t43611 + t45451 + F::cast_from(0.38342925953920749677e1_f64) * t43650 + t45453 + t45454;
    t45455
}
