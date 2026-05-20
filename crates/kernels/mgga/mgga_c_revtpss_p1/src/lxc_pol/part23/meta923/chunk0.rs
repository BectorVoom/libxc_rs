//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2984/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2984<F: Float>(t19658: F, t4879: F, t4772: F, t6258: F, t23633: F, t4786: F, t23842: F, t1011: F, t1042: F, t1063: F, t11774: F, t11994: F, t15700: F, t15701: F, t15725: F, t16190: F, t16222: F, t1651: F, t1675: F, t18281: F, t19663: F, t23859: F, t23863: F, t23966: F, t247: F, t3116: F, t3127: F, t4834: F, t4837: F, t4872: F, t4915: F, t4919: F, t53320: F, t53322: F, t53332: F, t53473: F, t5825: F, t6302: F, t65689: F, t67269: F, t77513: F, t77579: F, t77584: F, t78785: F) -> (F, F, F, F) {
    let t79071 = t4879 * t19658;
    let t79084 = t4772 * t6258;
    let t79097 = t23633 * t4786;
    let t79101 = t23842 * t4786;
    let t79105 = -F::cast_from(0.42874018118069736972e-3_f64) * t11994 * t23859 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t4872 * t18281 * t1651 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t4872 * t5825 * t4772 + F::cast_from(0.85748036236139473944e-3_f64) * t15725 * t23863 + F::cast_from(0.23289590088828005269e-2_f64) * t1063 * t1042 * t53473 * t78785 - F::cast_from(0.34299214494455789578e-2_f64) * t16190 * t6302 + F::cast_from(0.42874018118069736972e-3_f64) * t79071 - F::cast_from(0.42874018118069736972e-2_f64) * t4834 * t19663 - t1011 * t4915 * t77579 / F::new(144.0) + t1011 * t4919 * t77584 / F::new(216.0) - F::cast_from(0.11433071498151929859e-2_f64) * t65689 + F::cast_from(0.12862205435420921092e-2_f64) * t15725 * t23966 + F::cast_from(0.12862205435420921092e-2_f64) * t4837 * t247 * t3116 * t79084 + F::cast_from(0.42874018118069736972e-3_f64) * t67269 * t1675 - t53320 * t53332 * t77513 / F::new(12.0) + F::new(7.0) / F::new(216.0) * t53320 * t53322 * t77513 - F::cast_from(0.7145669686344956162e-3_f64) * t11774 * t16222 * t79097 - F::cast_from(0.85748036236139473944e-3_f64) * t15700 * t15701 * t79101;
    (t79084, t79097, t79101, t79105)
}
