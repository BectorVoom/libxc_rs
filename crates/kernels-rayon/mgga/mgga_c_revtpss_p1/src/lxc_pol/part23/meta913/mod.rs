//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta913 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2944;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2945;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta913(t291: f64, t78132: f64, t78151: f64, t15400: f64, t1622: f64, t1634: f64, t19173: f64, t19227: f64, t19300: f64, t23755: f64, t23776: f64, t2938: f64, t41662: f64, t4647: f64, t4670: f64, t52430: f64, t6174: f64, t64055: f64, t64120: f64, t77886: f64, t77898: f64, t77911: f64, t77923: f64, t77935: f64, t77947: f64, t77961: f64, t77974: f64, t78094: f64, t78096: f64, t78099: f64, t78108: f64, t78111: f64, t946: f64, t954: f64, t955: f64, t974: f64, t23754: f64, t2970: f64, t11528: f64, t23767: f64, t2874: f64, t4632: f64, t6141: f64, t11409: f64, t11450: f64, t11466: f64, t15350: f64, t15413: f64, t19282: f64, t19304: f64, t19307: f64, t19311: f64, t23705: f64, t2943: f64, t2968: f64, t41667: f64, t41740: f64, t41742: f64, t4669: f64, t4690: f64, t4707: f64, t4712: f64, t52642: f64, t6177: f64, t6209: f64, t63997: f64, t64125: f64, t953: f64, t1610: f64, t19127: f64, t11294: f64, t23770: f64, t1609: f64, t2924: f64, t63650: f64, t23694: f64, t3014: f64, t11461: f64, t11507: f64, t15406: f64, t1633: f64, t19279: f64, t19283: f64, t19303: f64, t19310: f64, t23451: f64, t23714: f64, t23717: f64, t23764: f64, t2987: f64, t3012: f64, t41238: f64, t41658: f64, t41759: f64, t4652: f64, t4674: f64, t52825: f64, t64060: f64, t64072: f64, t64319: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78154, t78155) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2944(t291, t78132, t78151, t15400, t1622, t1634, t19173, t19227, t19300, t23755, t23776, t2938, t41662, t4647, t4670, t52430, t6174, t64055, t64120, t77886, t77898, t77911, t77923, t77935, t77947, t77961, t77974, t78094, t78096, t78099, t78108, t78111, t946, t954, t955, t974);
        let (t78192, t78195, t78196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2945(t23754, t2970, t11528, t23767, t2874, t4632, t6141, t11409, t11450, t11466, t15350, t15413, t19282, t19300, t19304, t19307, t19311, t23705, t23755, t23776, t2943, t2968, t41667, t41740, t41742, t4669, t4690, t4707, t4712, t52642, t6177, t6209, t63997, t64125, t953);
        let (t78201, t78203, t78206, t78240) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2946(t1610, t19127, t2874, t11294, t23770, t1609, t2924, t63650, t23694, t3014, t11461, t11507, t15406, t1633, t19279, t19283, t19303, t19310, t23451, t23714, t23717, t23764, t2987, t3012, t41238, t41658, t41759, t4652, t4674, t4707, t52825, t64060, t64072, t64319, t972);
    (t78154, t78155, t78192, t78195, t78196, t78201, t78203, t78206, t78240)
}
