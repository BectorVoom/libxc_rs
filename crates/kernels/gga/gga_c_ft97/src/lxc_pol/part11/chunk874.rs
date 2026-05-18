//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 874/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk874<F: Float>(t37930: F, t1619: F, t7934: F, t1597: F, t62: F, t66: F, t22547: F, t1620: F, t6: F, t7984: F, t7988: F, t5517: F, t5544: F) -> (F, F, F, F, F, F) {
    let t37931 = F::new(1.0) / t37930;
    let t37935 = t1619 * t7934;
    let t37939 = t1597 * t62;
    let t37940 = t37939 * t66;
    let t37941 = t22547 * t37940;
    let t37943 = t7984 * t6 * t1620;
    let t37947 = t7988 * t6 * t1620;
    let t37952 = t5517 * t5544;
    (t37931, t37935, t37941, t37943, t37947, t37952)
}
