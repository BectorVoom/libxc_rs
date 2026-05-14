//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 935/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk935<F: Float>(t2469: F, t35678: F, t33274: F, t3972: F, t33489: F, t3977: F, t2568: F, t6187: F, t6940: F, t1449: F, t27889: F, t1173: F, t1403: F, t141552: F, t193: F, t27947: F, t33245: F, t33248: F, t33582: F, t33584: F, t35263: F, t5996: F, t6192: F, t6745: F, t6838: F, t7437: F) -> (F, F, F, F, F, F) {
    let t151353 = t2469 * t35678;
    let t151355 = t33274 * t3972;
    let t151357 = t3977 * t33489;
    let t151362 = t2568 * t6187 * t6940;
    let t151365 = t2568 * t1449 * t27889;
    let t151380 = t7437 * t27947 / 6.0 - 2.0 * t151353 - 2.0 * t151355 - 2.0 * t151357 + t5996 * t35263 / 3.0 + 8.0 * t151362 + 8.0 * t151365 + t6745 * t33245 - 2.0 / 3.0 * t6745 * t33248 + t1403 * t193 * t33582 * t1173 / 6.0 + t6745 * t33584 / 6.0 + t1403 * t193 * t6838 * t6192 / 3.0 - t141552;
    (t151353, t151355, t151357, t151362, t151365, t151380)
}
