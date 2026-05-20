//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2143;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2144;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2145;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2146;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2147;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2148;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta623<F: Float>(t25260: F, t4368: F, t820: F, t844: F, t14914: F, t25270: F, t14919: F, t14904: F, t27261: F, t14900: F, t4462: F, t92951: F, t14788: F, t14682: F, t14804: F, t14793: F, t92952: F, t92956: F, t92963: F, t92966: F, t92969: F, t27253: F, t9775: F, t14833: F, t240: F, t2661: F, t7043: F, t14853: F, t7045: F, t14857: F, t25234: F, t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F, t92971: F, t92976: F, t92979: F, t14701: F, t92955: F, t14707: F, t241: F, t93060: F, t14896: F, t4447: F, t14874: F, t14746: F, t7025: F, t14769: F, t14727: F, t25227: F, t4430: F, t93034: F, t92991: F, t14861: F, t92989: F, t1565: F, t93066: F, t25222: F, t4345: F, t4349: F, t93072: F, t14910: F, t14678: F, t14673: F, t14688: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98937, t98940, t98943, t98945, t98947, t98949) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2143::<F>(t25260, t4368, t820, t844, t14914, t25270, t14919, t14904, t27261, t14900, t4462, t92951);
        let t98959 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2144::<F>(t98949, t14788, t25270, t14682, t14804, t27261, t14793, t92952, t92956, t98937, t98940, t98943, t98945, t98947);
        let (t98960, t98961, t98962, t98964, t98968, t98970, t98972) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2145::<F>(t92963, t92966, t92969, t27253, t9775, t14833, t240, t2661, t7043, t14853, t7045, t14857, t25234);
        let t98981 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2146::<F>(t98972, t25240, t2710, t4371, t10744, t4353, t7028, t92971, t92976, t92979, t98960, t98961, t98962, t98964, t98968, t98970);
        let (t98984, t98985, t98989, t98992, t98993, t98995) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2147::<F>(t14701, t92955, t14707, t25270, t241, t820, t93060, t14896, t4447, t92951, t14874, t14746, t7025);
        let t99008 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2148::<F>(t14769, t7045, t14727, t25227, t2661, t4430, t93034, t92991, t14861, t92989, t98984, t98985, t98989, t98992, t98993, t98995);
        let (t99009, t99012, t99013, t99015, t99017, t99020, t99021) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2149::<F>(t1565, t93066, t25222, t4345, t4349, t93072, t14910, t25270, t14678, t14673, t92955, t14688);
    (t98959, t98981, t99008, t99009, t99012, t99013, t99015, t99017, t99020, t99021)
}
