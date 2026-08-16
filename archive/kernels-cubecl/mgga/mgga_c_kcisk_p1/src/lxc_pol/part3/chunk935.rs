//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 935/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk935<F: Float>(t1305: F, t4155: F, t392: F, t495: F, t20: F, t389: F, t4001: F, t1294: F, t3981: F, t1293: F, t4000: F, t3993: F) -> (F, F, F, F, F, F, F) {
    let t13851 = t4155 * t1305;
    let t13854 = F::cast_from(1.0_f64) / t392 / t495;
    let t13855 = t13854 * t20;
    let t13856 = t389 * t13855;
    let t13859 = t4001 * t1305;
    let t13861 = t1294 * t3981;
    let t13863 = t1293 * t4000;
    let t13866 = t3993 * t1305;
    (t13851, t13854, t13856, t13859, t13861, t13863, t13866)
}
