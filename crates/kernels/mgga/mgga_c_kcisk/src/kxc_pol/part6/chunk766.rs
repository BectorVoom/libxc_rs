//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 766/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk766<F: Float>(t116: F, t15660: F, t20: F, t3042: F, t268: F, t1120: F, t1123: F, t3411: F, t397: F, t3410: F, t3417: F, t1111: F, t1119: F, t1125: F, t119: F, t15631: F, t15637: F, t15643: F, t275: F, t3392: F, t3399: F, t3406: F, t3413: F, t3419: F, t5821: F, t918: F) -> F {
    let t15661 = t15660 * t116;
    let t15668 = t3042 * t20;
    let t15669 = t268 * t15668;
    let t15680 = t1120 * t1120;
    let t15681 = F::new(1.0) / t15680;
    let t15682 = t3411 * t1123;
    let t15684 = t397 * t15681 * t15682;
    let t15689 = t397 * t3410 * t1123 * t3417;
    let t15692 = -F::cast_from(0.5397236614853195164e-1_f64) * t1119 * t15631 - F::cast_from(0.37780656303972366147e0_f64) * t3392 * t918 * t275 - F::cast_from(0.16191709844559585492e0_f64) * t15637 * t1125 + F::cast_from(0.12593552101324122049e1_f64) * t1111 * t3042 * t275 + F::cast_from(0.75561312607944732295e0_f64) * t15643 * t1125 + F::cast_from(0.5397236614853195164e-1_f64) * t15661 * t119 * t275 - F::cast_from(0.18190686368579287404e1_f64) * t268 * t5821 * t275 - F::cast_from(0.12593552101324122049e1_f64) * t15669 * t1125 + F::cast_from(0.37780656303972366147e0_f64) * t3406 * t3419 - F::cast_from(0.16191709844559585492e0_f64) * t3399 * t3419 + F::cast_from(0.32383419689119170984e0_f64) * t3399 * t3413 - F::cast_from(0.75561312607944732295e0_f64) * t3406 * t3413 - F::cast_from(0.32383419689119170984e0_f64) * t1119 * t15684 + F::cast_from(0.32383419689119170984e0_f64) * t1119 * t15689;
    t15692
}
