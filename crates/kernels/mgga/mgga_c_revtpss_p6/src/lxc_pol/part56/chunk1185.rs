//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1185/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1185<F: Float>(t131594: F, t73: F, t29127: F, t8937: F, t33421: F, t34991: F, t12915: F, t247: F, t33398: F, t34929: F, t1042: F, t1122: F, t124573: F, t124578: F, t124584: F, t124601: F, t124755: F, t1248: F, t124825: F, t124945: F, t125009: F, t1287: F, t131439: F, t131591: F, t131595: F, t1797: F, t29159: F, t31993: F, t33491: F, t34908: F, t34934: F, t3596: F, t3626: F, t5296: F, t5299: F, t5385: F, t5480: F, t8938: F) -> (F, F, F) {
    let t131699 = t131594 * t73;
    let t131703 = t8937 * t29127;
    let t131706 = t34991 * t33421;
    let t131710 = t33398 * t247 * t12915 * t34929;
    let t131725 = F::cast_from(0.82638509353446690713e-4_f64) * t124825 - F::cast_from(0.22312397525430606492e-2_f64) * t124573 * t31993 * t5385 + F::cast_from(0.37645955677973955999e-3_f64) * t124755 * t3626 * t34934 * t1122 + F::cast_from(0.22847895066040941046e1_f64) * t125009 * t34908 * t1248 * t1287 + F::cast_from(0.34271842599061411569e1_f64) * t124945 * t131699 * t29159 + F::cast_from(0.11423947533020470523e1_f64) * t131703 * t33491 - F::cast_from(0.10038921514126388266e-2_f64) * t131706 + F::cast_from(0.37645955677973955999e-3_f64) * t131710 - F::cast_from(0.3718732920905101082e-3_f64) * t124601 * t1797 + F::cast_from(0.37187329209051010821e-3_f64) * t124578 * t1042 * t5296 * t131439 - F::cast_from(0.37187329209051010821e-3_f64) * t124584 * t5299 - F::cast_from(0.11423947533020470523e1_f64) * t8938 * t131591 * t3596 * t131595 * t5480;
    (t131699, t131703, t131725)
}
