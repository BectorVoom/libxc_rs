//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1021/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1021<F: Float>(t338: F, t30738: F, t1320: F, t1310: F, t2168: F, t8048: F, t13831: F, t2075: F, t8054: F, t3937: F, t7736: F, t13504: F, t1309: F, t13873: F, t20169: F, t20185: F, t2170: F, t26008: F, t26430: F, t26471: F, t26485: F, t26490: F, t30536: F, t30540: F, t30544: F, t30548: F, t3935: F, t6157: F, t8050: F, t8056: F) -> F {
    let t400 = F::new(0.0) < t338;
    let t30740 = piecewise3::<f64>(t400, t30738, -t30738);
    let t30741 = t1320 * t30740;
    let t30742 = t1310 * t30741;
    let t30747 = t8048 * t2168;
    let t30748 = t13831 * t30747;
    let t30749 = t1310 * t30748;
    let t30758 = t2075 * t8054;
    let t30759 = t3937 * t30758;
    let t30762 = t7736 * t2168;
    let t30763 = t13504 * t30762;
    let t30769 = F::new(0.53972366148531951639e-1) * t26430 + F::new(0.71963154864709268853e-1) * t3935 * t30536 + F::new(0.10794473229706390328e0) * t1309 * t30540 - F::new(0.1439263097294185377e0) * t1309 * t30544 + F::new(0.55971342672551653552e-1) * t1309 * t30548 - F::new(0.5397236614853195164e-1) * t1309 * t30742 + F::new(0.32383419689119170984e0) * t6157 * t8050 - F::new(0.32383419689119170984e0) * t1309 * t30749 - F::new(0.16191709844559585492e0) * t26008 * t2170 - F::new(0.35981577432354634426e-1) * t20169 + F::new(0.35981577432354634426e-1) * t20185 - F::new(0.16191709844559585492e0) * t6157 * t8056 - F::new(0.53972366148531951639e-1) * t3935 * t30759 - F::new(0.71963154864709268852e-1) * t3935 * t30763 + t13873 - F::new(0.53972366148531951639e-1) * t26471 - F::new(0.10794473229706390328e0) * t26485 + F::new(0.10794473229706390328e0) * t26490;
    t30769
}
