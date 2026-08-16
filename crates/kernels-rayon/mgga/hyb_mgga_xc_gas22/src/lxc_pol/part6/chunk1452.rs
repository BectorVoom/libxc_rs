//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1452/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1452(t10850: f64, t1115: f64, t11258: f64, t11617: f64, t1175: f64, t1514: f64, t1528: f64, t1563: f64, t2447: f64, t2628: f64, t27847: f64, t2817: f64, t28697: f64, t2944: f64, t29533: f64, t30373: f64, t30387: f64, t30408: f64, t30431: f64, t30439: f64, t30451: f64, t30512: f64, t30530: f64, t30548: f64, t30610: f64, t30648: f64, t30681: f64, t30719: f64, t30757: f64, t30799: f64, t30836: f64, t30867: f64, t30902: f64, t30939: f64, t30968: f64, t30999: f64, t31030: f64, t31065: f64, t31105: f64, t31139: f64, t31176: f64, t31205: f64, t31236: f64, t31271: f64, t31303: f64, t31337: f64, t31370: f64, t31405: f64, t31441: f64, t31477: f64, t31511: f64, t31545: f64, t31575: f64, t31612: f64, t31655: f64, t31685: f64, t338: f64, t3656: f64, t3792: f64, t4222: f64, t436: f64, t4458: f64, t4485: f64, t4583: f64, t500: f64, t541: f64, t930: f64, t9314: f64, t9411: f64, t9813: f64) -> f64 {
    let t31692 = t27847 + t28697 + t29533 * t338 + 2.0_f64 * t10850 * t930 + t4222 * t2447 + t30373 * t436 + 2.0_f64 * t9314 * t1514 + t2628 * t4458 + (t30387 + t30408 + t30431 + t30439 + t30451 + t30512 + t30530 + t30548) * t541 + 2.0_f64 * t11258 * t1175 + t4485 * t2944 + 2.0_f64 * t9411 * t1563 + 4.0_f64 * t3656 * t3792 + 2.0_f64 * t1528 * t9813 + t2817 * t4583 + 2.0_f64 * t1115 * t11617 + t500 * (t30648 + t31477 + t31205 + t30902 + t30610 + t31105 + t31176 + t31236 + t31685 + t30799 + t31370 + t31405 + t31337 + t31065 + t31271 + t31139 + t31303 + t31655 + t31575 + t31441 + t31030 + t30719 + t30939 + t30836 + t30757 + t30681 + t30999 + t31511 + t31612 + t31545 + t30867 + t30968);
    t31692
}
