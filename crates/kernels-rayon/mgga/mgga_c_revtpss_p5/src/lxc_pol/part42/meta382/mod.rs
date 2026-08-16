//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1261;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1262;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1263;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta382(t15421: f64, t4636: f64, t6110: f64, t934: f64, t2924: f64, t1610: f64, t4631: f64, t2874: f64, t6145: f64, t11299: f64, t6142: f64, t2926: f64, t6141: f64, t11466: f64, t11507: f64, t19294: f64, t19297: f64, t19300: f64, t19304: f64, t19307: f64, t19311: f64, t19315: f64, t2987: f64, t3012: f64, t19172: f64, t19253: f64, t19293: f64, t300: f64, t6350: f64, t999: f64, t3269: f64, t342: f64, t6343: f64, t11133: f64, t11134: f64, t15127: f64, t15189: f64, t15638: f64, t15639: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64, t996: f64, t6392: f64, t1079: f64, t1097: f64, t16305: f64, t1652: f64, t16600: f64, t3052: f64, t3264: f64, t4747: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4778: f64, t5016: f64, t6351: f64, t6393: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19317, t19320, t19323, t19326, t19329, t19330) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1261(t15421, t4636, t6110, t934, t2924, t1610, t4631, t2874, t6145, t11299, t6142, t2926, t6141);
        let (t19333, t19334) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1262(t19330, t934, t2924, t11466, t11507, t19294, t19297, t19300, t19304, t19307, t19311, t19315, t19317, t19320, t19323, t19326, t19329, t2987, t3012);
        let (t19337, t19342, t19351, t19380) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1263(t19172, t19253, t19293, t19334, t300, t6350, t999, t3269, t342, t6343, t11133, t11134, t15127, t15189, t15638, t15639, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let t19390 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1264(t19380, t996, t6392, t999, t1079, t1097, t16305, t1652, t16600, t19342, t19351, t3052, t3264, t4747, t4752, t4758, t4764, t4773, t4778, t5016, t6351, t6393, t995);
    (t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19380, t19390)
}
