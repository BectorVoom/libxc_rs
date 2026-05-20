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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1733;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1734;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1735;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1736;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1737;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1738;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta567<F: Float>(t68255: F, t81156: F, t81158: F, t89824: F, t89828: F, t89832: F, t89839: F, t89843: F, t89847: F, t89851: F, t89855: F, t43776: F, t87145: F, t12305: F, t128: F, t12256: F, t3360: F, t12268: F, t1120: F, t1121: F, t87126: F, t44307: F, t56236: F, t68257: F, t68399: F, t81230: F, t81232: F, t81234: F, t81236: F, t459: F, t1211: F, t12628: F, t1274: F, t1277: F, t13182: F, t1770: F, t1828: F, t1829: F, t20753: F, t20756: F, t21394: F, t24509: F, t24519: F, t24525: F, t24616: F, t24866: F, t25022: F, t3567: F, t3737: F, t495: F, t5220: F, t5225: F, t5417: F, t6573: F, t6574: F, t6588: F, t6702: F, t6703: F, t6744: F, t6745: F, t72802: F, t82147: F, t89808: F, t1210: F, t1774: F, t17986: F, t17987: F, t18059: F, t20697: F, t21621: F, t24514: F, t24524: F, t24633: F, t24892: F, t24900: F, t25016: F, t5251: F, t6580: F, t6587: F, t72874: F, t84952: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t89857, t89863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1732::<F>(t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855, t43776, t87145);
        let t89865 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1733::<F>(t12305, t128, t89863);
        let (t89867, t89869) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1734::<F>(t12256, t87145, t128, t3360);
        let (t89871, t89873) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1735::<F>(t12268, t87145, t1120, t128);
        let (t89875, t89877) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1736::<F>(t1121, t87126, t1120, t128);
        let t89881 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1737::<F>(t44307, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
        let (t89883, t89888) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1738::<F>(t459, t89857, t89881, t1211, t12628, t1274, t1277, t13182, t1770, t1828, t1829, t20753, t20756, t21394, t24509, t24519, t24525, t24616, t24866, t25022, t3567, t3737, t495, t5220, t5225, t5417, t6573, t6574, t6588, t6702, t6703, t6744, t6745, t72802, t82147, t89808);
        let t89930 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1739::<F>(t1210, t1277, t13182, t1774, t17986, t17987, t18059, t1828, t1829, t20697, t20753, t21394, t21621, t24514, t24519, t24524, t24633, t24892, t24900, t25016, t25022, t5220, t5251, t5417, t6574, t6580, t6587, t6588, t6744, t6745, t72874, t84952);
    (t89863, t89865, t89867, t89869, t89871, t89873, t89875, t89877, t89883, t89888, t89930)
}
