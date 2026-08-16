//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta913 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2944;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2945;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta913<F: Float>(t291: F, t78132: F, t78151: F, t15400: F, t1622: F, t1634: F, t19173: F, t19227: F, t19300: F, t23755: F, t23776: F, t2938: F, t41662: F, t4647: F, t4670: F, t52430: F, t6174: F, t64055: F, t64120: F, t77886: F, t77898: F, t77911: F, t77923: F, t77935: F, t77947: F, t77961: F, t77974: F, t78094: F, t78096: F, t78099: F, t78108: F, t78111: F, t946: F, t954: F, t955: F, t974: F, t23754: F, t2970: F, t11528: F, t23767: F, t2874: F, t4632: F, t6141: F, t11409: F, t11450: F, t11466: F, t15350: F, t15413: F, t19282: F, t19304: F, t19307: F, t19311: F, t23705: F, t2943: F, t2968: F, t41667: F, t41740: F, t41742: F, t4669: F, t4690: F, t4707: F, t4712: F, t52642: F, t6177: F, t6209: F, t63997: F, t64125: F, t953: F, t1610: F, t19127: F, t11294: F, t23770: F, t1609: F, t2924: F, t63650: F, t23694: F, t3014: F, t11461: F, t11507: F, t15406: F, t1633: F, t19279: F, t19283: F, t19303: F, t19310: F, t23451: F, t23714: F, t23717: F, t23764: F, t2987: F, t3012: F, t41238: F, t41658: F, t41759: F, t4652: F, t4674: F, t52825: F, t64060: F, t64072: F, t64319: F, t972: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t78154, t78155) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2944::<F>(t291, t78132, t78151, t15400, t1622, t1634, t19173, t19227, t19300, t23755, t23776, t2938, t41662, t4647, t4670, t52430, t6174, t64055, t64120, t77886, t77898, t77911, t77923, t77935, t77947, t77961, t77974, t78094, t78096, t78099, t78108, t78111, t946, t954, t955, t974);
        let (t78192, t78195, t78196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2945::<F>(t23754, t2970, t11528, t23767, t2874, t4632, t6141, t11409, t11450, t11466, t15350, t15413, t19282, t19300, t19304, t19307, t19311, t23705, t23755, t23776, t2943, t2968, t41667, t41740, t41742, t4669, t4690, t4707, t4712, t52642, t6177, t6209, t63997, t64125, t953);
        let (t78201, t78203, t78206, t78240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2946::<F>(t1610, t19127, t2874, t11294, t23770, t1609, t2924, t63650, t23694, t3014, t11461, t11507, t15406, t1633, t19279, t19283, t19303, t19310, t23451, t23714, t23717, t23764, t2987, t3012, t41238, t41658, t41759, t4652, t4674, t4707, t52825, t64060, t64072, t64319, t972);
    (t78154, t78155, t78192, t78195, t78196, t78201, t78203, t78206, t78240)
}
