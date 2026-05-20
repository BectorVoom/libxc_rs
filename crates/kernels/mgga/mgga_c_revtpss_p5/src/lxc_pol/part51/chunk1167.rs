//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1167/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1167<F: Float>(t1035: F, t1695: F, t1042: F, t1043: F, t1045: F, t1078: F, t1089: F, t1096: F, t120275: F, t120305: F, t120307: F, t120313: F, t120400: F, t120425: F, t120507: F, t120538: F, t120671: F, t120708: F, t120709: F, t1646: F, t1669: F, t247: F, t27441: F, t3116: F, t31891: F, t31934: F, t31935: F, t31959: F, t33808: F, t33811: F, t33812: F, t33825: F, t4786: F, t4866: F, t4906: F, t5015: F, t8502: F) -> F {
    let t126819 = t1035 * t1695;
    let t126828 = F::cast_from(0.56468933516960933998e-3_f64) * t120305 * t120307 * t1646 * t1043 * t1045 - F::cast_from(0.56468933516960933998e-3_f64) * t120313 * t120307 * t4906 + F::cast_from(0.7437465841810202164e-3_f64) * t120275 * t1042 * t33825 * t4786 - F::cast_from(0.11156198762715303246e-2_f64) * t120708 * t1042 * t1669 * t120709 + F::cast_from(0.34694512752820797848e1_f64) * t120400 * t27441 - F::cast_from(0.37645955677973955998e-3_f64) * t120538 + F::cast_from(0.11423947533020470523e1_f64) * t120671 * t33808 + F::cast_from(0.11423947533020470523e1_f64) * t31934 * t31935 * t4866 * t1089 - F::cast_from(0.28234466758480466999e-3_f64) * t8502 * t247 * t3116 * t1078 * t5015 + F::cast_from(0.11423947533020470523e1_f64) * t120425 * t33812 + F::cast_from(0.17347256376410398924e1_f64) * t120507 * t126819 * t1043 * t1089 - F::cast_from(0.34271842599061411569e1_f64) * t31891 * t31959 * t33811 * t1096;
    t126828
}
