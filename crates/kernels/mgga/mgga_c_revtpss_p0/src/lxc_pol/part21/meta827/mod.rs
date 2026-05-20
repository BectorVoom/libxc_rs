//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta827 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3080;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3081;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3082;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta827<F: Float>(t3555: F, t488: F, t17807: F, t460: F, t1276: F, t5245: F, t13181: F, t1828: F, t12627: F, t12626: F, t1769: F, t487: F, t56176: F, t56183: F, t43830: F, t43832: F, t44865: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F, t1770: F, t3727: F, t1210: F, t1211: F, t12606: F, t12622: F, t12630: F, t12646: F, t12695: F, t1277: F, t1295: F, t13170: F, t13173: F, t13177: F, t1775: F, t17964: F, t17973: F, t17974: F, t17988: F, t18090: F, t1829: F, t21389: F, t34934: F, t34964: F, t3572: F, t3575: F, t3732: F, t3790: F, t45464: F, t45568: F, t5220: F, t5423: F) -> (F, F, F, F, F) {
        let (t56294, t56303, t56310, t56314, t56315, t56327, t56331, t56332) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3080::<F>(t3555, t488, t17807, t460, t1276, t5245, t13181, t1828, t12627, t12626, t1769, t487);
        let t56354 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3081::<F>(t56176, t56183, t43830, t43832, t44865, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t56375 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3082::<F>(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t56376, t56390) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3083::<F>(t56354, t56375, t1770, t3727, t1210, t1211, t12606, t12622, t12630, t12646, t12695, t1277, t1295, t13170, t13173, t13177, t1775, t17964, t17973, t17974, t17988, t18090, t1829, t21389, t34934, t34964, t3572, t3575, t3732, t3790, t45464, t45568, t5220, t5245, t5423, t56294, t56303, t56310, t56314, t56315, t56327, t56332);
    (t56315, t56327, t56331, t56376, t56390)
}
