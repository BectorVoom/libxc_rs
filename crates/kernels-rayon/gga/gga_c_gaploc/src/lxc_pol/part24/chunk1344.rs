//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1344/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1344(t19933: f64, t8054: f64, t2592: f64, t8854: f64, t10283: f64, t1651: f64, t2969: f64, t7817: f64, t10800: f64, t2208: f64, t32102: f64, t32783: f64, t32836: f64, t32873: f64, t32919: f64, t32947: f64, t32980: f64, t33025: f64, t33061: f64, t33102: f64, t33146: f64, t33165: f64, t33200: f64, t33241: f64, t33275: f64, t33322: f64, t33369: f64, t33413: f64, t33444: f64, t33487: f64, t33540: f64, t33582: f64, t33620: f64, t33663: f64, t33696: f64, t33729: f64, t33752: f64, t33777: f64, t33815: f64, t33849: f64, t33887: f64, t33902: f64, t33944: f64, t33952: f64, t33955: f64, t33958: f64, t33961: f64, t33963: f64, t33966: f64, t33968: f64, t33970: f64, t33973: f64, t33974: f64, t33977: f64, t748: f64) -> (f64, f64, f64) {
    let t33979 = 6.0_f64 * t19933 * t8054;
    let t33980 = t2592 * t8854;
    let t33981 = t10283 * t1651;
    let t33982 = t2969 * t7817;
    let t33983 = -t748 * (t33944 + t33902 + t33887 + t33849 + t33815 + t33777 + t33752 + t33729 + t33696 + t33663 + t33620 + t33582 + t33540 + t33487 + t33444 + t33413 + t33369 + t33322 + t33275 + t33241 + t33200 + t33165 + t32783 + t33061 + t32947 + t33025 + t32873 + t33146 + t32836 + t33102 + t32980 + t32919) + t32102 - t33952 + t33955 - t33958 - t33961 - t33963 - t33966 + t33968 + t33970 - t33973 + t33974 - t10800 * t2208 + t33977 + t33979 - t33980 + t33981 - t33982;
    (t33979, t33981, t33983)
}
