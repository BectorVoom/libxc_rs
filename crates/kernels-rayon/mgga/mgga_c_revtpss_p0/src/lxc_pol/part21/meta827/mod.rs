//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta827 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3080;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3081;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3082;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta827(t3555: f64, t488: f64, t17807: f64, t460: f64, t1276: f64, t5245: f64, t13181: f64, t1828: f64, t12627: f64, t12626: f64, t1769: f64, t487: f64, t56176: f64, t56183: f64, t43830: f64, t43832: f64, t44865: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64, t1770: f64, t3727: f64, t1210: f64, t1211: f64, t12606: f64, t12622: f64, t12630: f64, t12646: f64, t12695: f64, t1277: f64, t1295: f64, t13170: f64, t13173: f64, t13177: f64, t1775: f64, t17964: f64, t17973: f64, t17974: f64, t17988: f64, t18090: f64, t1829: f64, t21389: f64, t34934: f64, t34964: f64, t3572: f64, t3575: f64, t3732: f64, t3790: f64, t45464: f64, t45568: f64, t5220: f64, t5423: f64) -> (f64, f64, f64, f64, f64) {
        let (t56294, t56303, t56310, t56314, t56315, t56327, t56331, t56332) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3080(t3555, t488, t17807, t460, t1276, t5245, t13181, t1828, t12627, t12626, t1769, t487);
        let t56354 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3081(t56176, t56183, t43830, t43832, t44865, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t56375 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3082(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t56376, t56390) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3083(t56354, t56375, t1770, t3727, t1210, t1211, t12606, t12622, t12630, t12646, t12695, t1277, t1295, t13170, t13173, t13177, t1775, t17964, t17973, t17974, t17988, t18090, t1829, t21389, t34934, t34964, t3572, t3575, t3732, t3790, t45464, t45568, t5220, t5245, t5423, t56294, t56303, t56310, t56314, t56315, t56327, t56332);
    (t56315, t56327, t56331, t56376, t56390)
}
