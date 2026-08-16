//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2161;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta523(t15609: f64, t16432: f64, t15604: f64, t1089: f64, t1668: f64, t3259: f64, t15780: f64, t4983: f64, t3075: f64, t5004: f64, t359: f64, t4930: f64, t999: f64, t1043: f64, t4757: f64, t3291: f64, t4772: f64, t1678: f64, t3133: f64, t15957: f64, t4976: f64, t1024: f64, t1087: f64, t11782: f64, t11788: f64, t12122: f64, t12127: f64, t12149: f64, t16427: f64, t1685: f64, t1692: f64, t3043: f64, t3223: f64, t3278: f64, t3287: f64, t3299: f64, t3313: f64, t4954: f64, t4961: f64, t4981: f64, t4988: f64, t5005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16433, t16436, t16440, t16443, t16446, t16449) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2161(t15609, t16432, t15604, t1089, t1668, t3259, t15780, t4983, t3075, t5004, t359, t4930);
        let (t16450, t16458, t16461, t16465, t16468, t16475) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2162(t16449, t999, t1043, t1089, t4757, t3291, t4772, t1678, t3133, t15957, t4976, t1024, t1087, t11782, t11788, t12122, t12127, t12149, t16427, t16433, t16436, t16440, t16443, t16446, t1685, t1692, t3043, t3223, t3278, t3287, t3299, t3313, t4954, t4961, t4981, t4988, t5005);
    (t16433, t16436, t16440, t16443, t16446, t16449, t16450, t16458, t16461, t16465, t16468, t16475)
}
