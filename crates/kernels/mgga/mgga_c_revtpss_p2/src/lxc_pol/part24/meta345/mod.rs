//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1198;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1199;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1200;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1201;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta345<F: Float>(t11534: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t291: F, t15123: F, t23472: F, t23476: F, t23493: F, t23496: F, t23508: F, t23511: F, t11479: F, t11480: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F, t23536: F, t23538: F, t23541: F, t23543: F, t964: F, t973: F, t981: F, t1621: F, t6157: F, t954: F, t23451: F, t11509: F, t11507: F, t15104: F, t15413: F, t1622: F, t19173: F, t23461: F, t23463: F, t23465: F, t23469: F, t23549: F, t23552: F, t23564: F, t23567: F, t2968: F, t3012: F, t4647: F, t6158: F, t6174: F, t6190: F, t965: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23663, t23665) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1198::<F>(t11534, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505, t291);
        let t23680 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1199::<F>(t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505, t23508, t23511);
        let t23693 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1200::<F>(t11479, t11480, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523, t23536, t23538, t23541, t23543);
        let t23694 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1201::<F>(t23680, t23693);
        let (t23696, t23698, t23705, t23706, t23711, t23714, t23717, t23720) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1202::<F>(t23694, t964, t973, t981, t1621, t6157, t954, t23451, t11509, t11507, t15104, t15413, t1622, t19173, t23461, t23463, t23465, t23469, t23549, t23552, t23564, t23567, t2968, t3012, t4647, t6158, t6174, t6190, t965);
    (t23663, t23665, t23694, t23696, t23698, t23705, t23706, t23711, t23714, t23717, t23720)
}
