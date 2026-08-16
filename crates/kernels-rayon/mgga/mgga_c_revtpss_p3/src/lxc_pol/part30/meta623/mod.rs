//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2143;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2144;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2145;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2146;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2147;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2148;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta623(t25260: f64, t4368: f64, t820: f64, t844: f64, t14914: f64, t25270: f64, t14919: f64, t14904: f64, t27261: f64, t14900: f64, t4462: f64, t92951: f64, t14788: f64, t14682: f64, t14804: f64, t14793: f64, t92952: f64, t92956: f64, t92963: f64, t92966: f64, t92969: f64, t27253: f64, t9775: f64, t14833: f64, t240: f64, t2661: f64, t7043: f64, t14853: f64, t7045: f64, t14857: f64, t25234: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64, t92971: f64, t92976: f64, t92979: f64, t14701: f64, t92955: f64, t14707: f64, t241: f64, t93060: f64, t14896: f64, t4447: f64, t14874: f64, t14746: f64, t7025: f64, t14769: f64, t14727: f64, t25227: f64, t4430: f64, t93034: f64, t92991: f64, t14861: f64, t92989: f64, t1565: f64, t93066: f64, t25222: f64, t4345: f64, t4349: f64, t93072: f64, t14910: f64, t14678: f64, t14673: f64, t14688: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98937, t98940, t98943, t98945, t98947, t98949) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2143(t25260, t4368, t820, t844, t14914, t25270, t14919, t14904, t27261, t14900, t4462, t92951);
        let t98959 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2144(t98949, t14788, t25270, t14682, t14804, t27261, t14793, t92952, t92956, t98937, t98940, t98943, t98945, t98947);
        let (t98960, t98961, t98962, t98964, t98968, t98970, t98972) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2145(t92963, t92966, t92969, t27253, t9775, t14833, t240, t2661, t7043, t14853, t7045, t14857, t25234);
        let t98981 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2146(t98972, t25240, t2710, t4371, t10744, t4353, t7028, t92971, t92976, t92979, t98960, t98961, t98962, t98964, t98968, t98970);
        let (t98984, t98985, t98989, t98992, t98993, t98995) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2147(t14701, t92955, t14707, t25270, t241, t820, t93060, t14896, t4447, t92951, t14874, t14746, t7025);
        let t99008 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2148(t14769, t7045, t14727, t25227, t2661, t4430, t93034, t92991, t14861, t92989, t98984, t98985, t98989, t98992, t98993, t98995);
        let (t99009, t99012, t99013, t99015, t99017, t99020, t99021) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2149(t1565, t93066, t25222, t4345, t4349, t93072, t14910, t25270, t14678, t14673, t92955, t14688);
    (t98959, t98981, t99008, t99009, t99012, t99013, t99015, t99017, t99020, t99021)
}
