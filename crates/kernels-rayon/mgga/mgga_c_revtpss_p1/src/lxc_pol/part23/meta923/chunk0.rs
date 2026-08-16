//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2984/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2984(t19658: f64, t4879: f64, t4772: f64, t6258: f64, t23633: f64, t4786: f64, t23842: f64, t1011: f64, t1042: f64, t1063: f64, t11774: f64, t11994: f64, t15700: f64, t15701: f64, t15725: f64, t16190: f64, t16222: f64, t1651: f64, t1675: f64, t18281: f64, t19663: f64, t23859: f64, t23863: f64, t23966: f64, t247: f64, t3116: f64, t3127: f64, t4834: f64, t4837: f64, t4872: f64, t4915: f64, t4919: f64, t53320: f64, t53322: f64, t53332: f64, t53473: f64, t5825: f64, t6302: f64, t65689: f64, t67269: f64, t77513: f64, t77579: f64, t77584: f64, t78785: f64) -> (f64, f64, f64, f64) {
    let t79071 = t4879 * t19658;
    let t79084 = t4772 * t6258;
    let t79097 = t23633 * t4786;
    let t79101 = t23842 * t4786;
    let t79105 = -0.42874018118069736972e-3_f64 * t11994 * t23859 - 0.42874018118069736972e-3_f64 * t3127 * t1042 * t4872 * t18281 * t1651 - 0.42874018118069736972e-3_f64 * t3127 * t1042 * t4872 * t5825 * t4772 + 0.85748036236139473944e-3_f64 * t15725 * t23863 + 0.23289590088828005269e-2_f64 * t1063 * t1042 * t53473 * t78785 - 0.34299214494455789578e-2_f64 * t16190 * t6302 + 0.42874018118069736972e-3_f64 * t79071 - 0.42874018118069736972e-2_f64 * t4834 * t19663 - t1011 * t4915 * t77579 / 144.0_f64 + t1011 * t4919 * t77584 / 216.0_f64 - 0.11433071498151929859e-2_f64 * t65689 + 0.12862205435420921092e-2_f64 * t15725 * t23966 + 0.12862205435420921092e-2_f64 * t4837 * t247 * t3116 * t79084 + 0.42874018118069736972e-3_f64 * t67269 * t1675 - t53320 * t53332 * t77513 / 12.0_f64 + 7.0_f64 / 216.0_f64 * t53320 * t53322 * t77513 - 0.7145669686344956162e-3_f64 * t11774 * t16222 * t79097 - 0.85748036236139473944e-3_f64 * t15700 * t15701 * t79101;
    (t79084, t79097, t79101, t79105)
}
