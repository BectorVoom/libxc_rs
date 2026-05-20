//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1261;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1262;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1263;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta382<F: Float>(t15421: F, t4636: F, t6110: F, t934: F, t2924: F, t1610: F, t4631: F, t2874: F, t6145: F, t11299: F, t6142: F, t2926: F, t6141: F, t11466: F, t11507: F, t19294: F, t19297: F, t19300: F, t19304: F, t19307: F, t19311: F, t19315: F, t2987: F, t3012: F, t19172: F, t19253: F, t19293: F, t300: F, t6350: F, t999: F, t3269: F, t342: F, t6343: F, t11133: F, t11134: F, t15127: F, t15189: F, t15638: F, t15639: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t996: F, t6392: F, t1079: F, t1097: F, t16305: F, t1652: F, t16600: F, t3052: F, t3264: F, t4747: F, t4752: F, t4758: F, t4764: F, t4773: F, t4778: F, t5016: F, t6351: F, t6393: F, t995: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19317, t19320, t19323, t19326, t19329, t19330) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1261::<F>(t15421, t4636, t6110, t934, t2924, t1610, t4631, t2874, t6145, t11299, t6142, t2926, t6141);
        let (t19333, t19334) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1262::<F>(t19330, t934, t2924, t11466, t11507, t19294, t19297, t19300, t19304, t19307, t19311, t19315, t19317, t19320, t19323, t19326, t19329, t2987, t3012);
        let (t19337, t19342, t19351, t19380) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1263::<F>(t19172, t19253, t19293, t19334, t300, t6350, t999, t3269, t342, t6343, t11133, t11134, t15127, t15189, t15638, t15639, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let t19390 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1264::<F>(t19380, t996, t6392, t999, t1079, t1097, t16305, t1652, t16600, t19342, t19351, t3052, t3264, t4747, t4752, t4758, t4764, t4773, t4778, t5016, t6351, t6393, t995);
    (t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19380, t19390)
}
