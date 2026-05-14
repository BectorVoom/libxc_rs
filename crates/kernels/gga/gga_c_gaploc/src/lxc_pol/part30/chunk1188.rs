//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1188/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1188<F: Float>(t2969: F, t7817: F, t10800: F, t2208: F, t32102: F, t32783: F, t32836: F, t32873: F, t32919: F, t32947: F, t32980: F, t33025: F, t33061: F, t33102: F, t33146: F, t33165: F, t33200: F, t33241: F, t33275: F, t33322: F, t33369: F, t33413: F, t33444: F, t33487: F, t33540: F, t33582: F, t33620: F, t33663: F, t33696: F, t33729: F, t33752: F, t33777: F, t33815: F, t33849: F, t33887: F, t33902: F, t33944: F, t33952: F, t33955: F, t33958: F, t33961: F, t33963: F, t33966: F, t33968: F, t33970: F, t33973: F, t33974: F, t33977: F, t33979: F, t33980: F, t33981: F, t748: F) -> (F,) {
    let t33982 = t2969 * t7817;
    let t33983 = -t748 * (t33944 + t33902 + t33887 + t33849 + t33815 + t33777 + t33752 + t33729 + t33696 + t33663 + t33620 + t33582 + t33540 + t33487 + t33444 + t33413 + t33369 + t33322 + t33275 + t33241 + t33200 + t33165 + t32783 + t33061 + t32947 + t33025 + t32873 + t33146 + t32836 + t33102 + t32980 + t32919) + t32102 - t33952 + t33955 - t33958 - t33961 - t33963 - t33966 + t33968 + t33970 - t33973 + t33974 - t10800 * t2208 + t33977 + t33979 - t33980 + t33981 - t33982;
    (t33983,)
}
