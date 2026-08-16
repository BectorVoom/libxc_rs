//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1733;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1734;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1735;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1736;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1737;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1738;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta567(t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64, t43776: f64, t87145: f64, t12305: f64, t128: f64, t12256: f64, t3360: f64, t12268: f64, t1120: f64, t1121: f64, t87126: f64, t44307: f64, t56236: f64, t68257: f64, t68399: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t459: f64, t1211: f64, t12628: f64, t1274: f64, t1277: f64, t13182: f64, t1770: f64, t1828: f64, t1829: f64, t20753: f64, t20756: f64, t21394: f64, t24509: f64, t24519: f64, t24525: f64, t24616: f64, t24866: f64, t25022: f64, t3567: f64, t3737: f64, t495: f64, t5220: f64, t5225: f64, t5417: f64, t6573: f64, t6574: f64, t6588: f64, t6702: f64, t6703: f64, t6744: f64, t6745: f64, t72802: f64, t82147: f64, t89808: f64, t1210: f64, t1774: f64, t17986: f64, t17987: f64, t18059: f64, t20697: f64, t21621: f64, t24514: f64, t24524: f64, t24633: f64, t24892: f64, t24900: f64, t25016: f64, t5251: f64, t6580: f64, t6587: f64, t72874: f64, t84952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89857, t89863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1732(t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855, t43776, t87145);
        let t89865 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1733(t12305, t128, t89863);
        let (t89867, t89869) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1734(t12256, t87145, t128, t3360);
        let (t89871, t89873) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1735(t12268, t87145, t1120, t128);
        let (t89875, t89877) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1736(t1121, t87126, t1120, t128);
        let t89881 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1737(t44307, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let (t89883, t89888) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1738(t459, t89857, t89881, t1211, t12628, t1274, t1277, t13182, t1770, t1828, t1829, t20753, t20756, t21394, t24509, t24519, t24525, t24616, t24866, t25022, t3567, t3737, t495, t5220, t5225, t5417, t6573, t6574, t6588, t6702, t6703, t6744, t6745, t72802, t82147, t89808);
        let t89930 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1739(t1210, t1277, t13182, t1774, t17986, t17987, t18059, t1828, t1829, t20697, t20753, t21394, t21621, t24514, t24519, t24524, t24633, t24892, t24900, t25016, t25022, t5220, t5251, t5417, t6574, t6580, t6587, t6588, t6744, t6745, t72874, t84952);
    (t89863, t89865, t89867, t89869, t89871, t89873, t89875, t89877, t89883, t89888, t89930)
}
