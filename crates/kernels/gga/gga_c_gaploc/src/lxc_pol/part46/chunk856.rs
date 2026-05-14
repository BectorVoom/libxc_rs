//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 856/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk856<F: Float>(t13146: F, t5676: F, t13077: F, t7712: F, t3040: F, t41468: F, t2536: F, t3431: F, t2009: F, t2021: F, t15498: F, t15499: F, t42944: F, t590: F, t23000: F, t33308: F, t9889: F) -> (F, F, F, F, F, F) {
    let t43817 = t5676 * t13146;
    let t43820 = 0.71500979903700853338e0 * t13077 * t7712;
    let t43822 = 0.35750489951850426669e0 * t41468 * t3040;
    let t43823 = t2536 * t3431;
    let t43825 = t2021 * t43823 * t2009;
    let t43830 = 0.61348681526273199482e1 * t15498 * t15499 * t42944 * t590;
    let t43832 = t23000 * t33308 * t9889;
    (t43817, t43820, t43822, t43825, t43830, t43832)
}
