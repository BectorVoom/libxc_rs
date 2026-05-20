//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3125/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3125<F: Float>(t343: F, t56: F, t816: F, t13026: F, t65: F, t12256: F, t1121: F, t1222: F, t1250: F, t12797: F, t12866: F, t13102: F, t17353: F, t17426: F, t17475: F, t17672: F, t17705: F, t17747: F, t17748: F, t247: F, t3584: F, t3625: F, t3626: F, t3718: F, t3719: F, t3720: F, t44548: F, t44559: F, t44571: F, t44583: F, t5056: F, t51959: F, t5312: F, t5373: F, t5381: F, t56149: F, t56201: F, t56219: F, t56561: F, t57498: F, t57508: F, t57520: F, t57534: F, t57536: F, t606: F) -> (F, F) {
    let t57548 = t56 * t343 * t816;
    let t57549 = t65 * t13026;
    let t57550 = t57549 * t12256;
    let t57555 = -F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t57498 * t1250 + F::cast_from(0.28582678745379824648e-3_f64) * t44548 - F::cast_from(0.63517063878621832552e-3_f64) * t5381 * t13102 - F::cast_from(0.25724410870841842184e-2_f64) * t57508 + F::cast_from(0.85748036236139473944e-3_f64) * t44559 + t1222 * t5312 * t56201 / F::new(12.0) + t1222 * t5312 * t56149 / F::new(6.0) - F::new(7.0) / F::new(216.0) * t1222 * t17475 * t56219 + F::cast_from(0.51448821741683684368e-2_f64) * t57520 * t247 * t3719 * t56561 - t5373 * t12797 / F::new(27.0) + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17353 * t1250 * t3584 * t1121 * t606 + F::cast_from(0.47637797908966374413e-3_f64) * t44571 + F::cast_from(0.17149607247227894789e-2_f64) * t57534 - F::cast_from(0.38586616306262763275e-2_f64) * t17747 * t3720 * t57536 * t17748 + F::cast_from(0.12862205435420921092e-2_f64) * t17426 * t17705 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t5056 * t17672 - F::new(7.0) / F::new(216.0) * t57548 * t57550 * t51959 - F::cast_from(0.85748036236139473944e-3_f64) * t44583;
    (t57548, t57555)
}
