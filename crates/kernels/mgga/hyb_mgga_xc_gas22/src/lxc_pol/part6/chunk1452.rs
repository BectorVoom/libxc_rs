//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1452/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1452<F: Float>(t10850: F, t1115: F, t11258: F, t11617: F, t1175: F, t1514: F, t1528: F, t1563: F, t2447: F, t2628: F, t27847: F, t2817: F, t28697: F, t2944: F, t29533: F, t30373: F, t30387: F, t30408: F, t30431: F, t30439: F, t30451: F, t30512: F, t30530: F, t30548: F, t30610: F, t30648: F, t30681: F, t30719: F, t30757: F, t30799: F, t30836: F, t30867: F, t30902: F, t30939: F, t30968: F, t30999: F, t31030: F, t31065: F, t31105: F, t31139: F, t31176: F, t31205: F, t31236: F, t31271: F, t31303: F, t31337: F, t31370: F, t31405: F, t31441: F, t31477: F, t31511: F, t31545: F, t31575: F, t31612: F, t31655: F, t31685: F, t338: F, t3656: F, t3792: F, t4222: F, t436: F, t4458: F, t4485: F, t4583: F, t500: F, t541: F, t930: F, t9314: F, t9411: F, t9813: F) -> F {
    let t31692 = t27847 + t28697 + t29533 * t338 + F::cast_from(2.0_f64) * t10850 * t930 + t4222 * t2447 + t30373 * t436 + F::cast_from(2.0_f64) * t9314 * t1514 + t2628 * t4458 + (t30387 + t30408 + t30431 + t30439 + t30451 + t30512 + t30530 + t30548) * t541 + F::cast_from(2.0_f64) * t11258 * t1175 + t4485 * t2944 + F::cast_from(2.0_f64) * t9411 * t1563 + F::cast_from(4.0_f64) * t3656 * t3792 + F::cast_from(2.0_f64) * t1528 * t9813 + t2817 * t4583 + F::cast_from(2.0_f64) * t1115 * t11617 + t500 * (t30648 + t31477 + t31205 + t30902 + t30610 + t31105 + t31176 + t31236 + t31685 + t30799 + t31370 + t31405 + t31337 + t31065 + t31271 + t31139 + t31303 + t31655 + t31575 + t31441 + t31030 + t30719 + t30939 + t30836 + t30757 + t30681 + t30999 + t31511 + t31612 + t31545 + t30867 + t30968);
    t31692
}
