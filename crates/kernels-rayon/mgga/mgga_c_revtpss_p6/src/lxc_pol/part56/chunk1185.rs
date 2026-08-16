//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1185/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1185(t131594: f64, t73: f64, t29127: f64, t8937: f64, t33421: f64, t34991: f64, t12915: f64, t247: f64, t33398: f64, t34929: f64, t1042: f64, t1122: f64, t124573: f64, t124578: f64, t124584: f64, t124601: f64, t124755: f64, t1248: f64, t124825: f64, t124945: f64, t125009: f64, t1287: f64, t131439: f64, t131591: f64, t131595: f64, t1797: f64, t29159: f64, t31993: f64, t33491: f64, t34908: f64, t34934: f64, t3596: f64, t3626: f64, t5296: f64, t5299: f64, t5385: f64, t5480: f64, t8938: f64) -> (f64, f64, f64) {
    let t131699 = t131594 * t73;
    let t131703 = t8937 * t29127;
    let t131706 = t34991 * t33421;
    let t131710 = t33398 * t247 * t12915 * t34929;
    let t131725 = 0.82638509353446690713e-4_f64 * t124825 - 0.22312397525430606492e-2_f64 * t124573 * t31993 * t5385 + 0.37645955677973955999e-3_f64 * t124755 * t3626 * t34934 * t1122 + 0.22847895066040941046e1_f64 * t125009 * t34908 * t1248 * t1287 + 0.34271842599061411569e1_f64 * t124945 * t131699 * t29159 + 0.11423947533020470523e1_f64 * t131703 * t33491 - 0.10038921514126388266e-2_f64 * t131706 + 0.37645955677973955999e-3_f64 * t131710 - 0.3718732920905101082e-3_f64 * t124601 * t1797 + 0.37187329209051010821e-3_f64 * t124578 * t1042 * t5296 * t131439 - 0.37187329209051010821e-3_f64 * t124584 * t5299 - 0.11423947533020470523e1_f64 * t8938 * t131591 * t3596 * t131595 * t5480;
    (t131699, t131703, t131725)
}
