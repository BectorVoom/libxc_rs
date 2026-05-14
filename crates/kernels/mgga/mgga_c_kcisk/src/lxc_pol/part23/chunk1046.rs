//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1046/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1046<F: Float>(t1504: F, t20980: F, t1512: F, t6339: F, t4226: F, t6382: F, t2271: F, t4306: F, t14187: F, t4229: F, t6370: F, t20942: F, t20944: F, t20947: F, t20949: F, t20951: F, t20953: F, t20955: F, t20959: F, t20962: F, t20965: F, t20967: F, t20970: F, t20973: F, t20976: F, t20978: F) -> (F, F, F, F, F, F, F) {
    let t20981 = t1504 * t20980;
    let t20983 = t1512 * t6339;
    let t20984 = t1504 * t20983;
    let t20986 = t6382 * t4226;
    let t20988 = t2271 * t4306;
    let t20990 = t14187 * t4229;
    let t20991 = t20990 * t6370;
    let t20993 = t20942 / 36.0 - t20944 / 192.0 + 11.0 / 27.0 * t20947 - t20949 / 18.0 + t20951 / 24.0 + t20953 / 18.0 + t20955 / 12.0 + t20959 / 6.0 - t20962 / 72.0 + t20965 / 54.0 - t20967 / 16.0 - t20970 / 288.0 - t20973 / 576.0 - 11.0 / 18.0 * t20976 + t20978 / 256.0 + 19.0 / 144.0 * t20981 - t20984 / 24.0 - t20986 / 24.0 - t20988 / 576.0 - t20991 / 64.0;
    (t20981, t20983, t20984, t20986, t20988, t20991, t20993)
}
