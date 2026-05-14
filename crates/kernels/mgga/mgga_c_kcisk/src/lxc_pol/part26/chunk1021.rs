//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1021/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1021<F: Float>(t1506: F, t27204: F, t26889: F, t6317: F, t4203: F, t26404: F, t6443: F, t6316: F, t27010: F, t6332: F, t6331: F, t21011: F, t2275: F, t27170: F, t27173: F, t27176: F, t27178: F, t27182: F, t27184: F, t27186: F, t27189: F, t27193: F, t27195: F, t27197: F, t27199: F, t27202: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27205 = t27204 * t1506;
    let t27207 = t6317 * t26889;
    let t27208 = t4203 * t27207;
    let t27210 = t6443 * t26404;
    let t27211 = t6316 * t27210;
    let t27213 = t6332 * t27010;
    let t27214 = t6331 * t27213;
    let t27216 = t21011 * t2275;
    let t27218 = -t27170 / 96.0 - t27173 / 576.0 - t27176 / 96.0 - t27178 / 576.0 - t27182 / 16.0 - t27184 / 8.0 - t27186 / 6.0 + t27189 / 192.0 + t27193 / 6.0 - t27195 / 18.0 - t27197 / 8.0 + t27199 / 3.0 + t27202 / 12.0 + t27205 / 256.0 + t27208 / 72.0 + t27211 / 54.0 + t27214 / 8.0 + t27216 / 128.0;
    (t27205, t27207, t27208, t27210, t27211, t27213, t27214, t27216, t27218)
}
