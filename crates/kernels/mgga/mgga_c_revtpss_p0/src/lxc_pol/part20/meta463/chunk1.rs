//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1761/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1761<F: Float>(t136: F, t4010: F, t220: F, t1399: F, t3945: F, t9816: F, t13804: F, t3889: F, t3934: F, t3936: F, t3937: F, t46416: F, t46655: F, t47216: F, t47221: F, t47223: F, t47227: F, t47229: F, t47231: F, t47235: F, t47239: F, t47245: F, t47248: F, t47249: F, t47259: F, t47262: F, t47264: F, t5673: F, t800: F, t9748: F, t9805: F, t9826: F, t9955: F, t9956: F) -> F {
    let t47273 = t4010 * t136;
    let t47274 = t47273 * t220;
    let t47277 = t9816 * t47274 * t3945 * t1399;
    let t47279 = -F::cast_from(0.16262400898971305032e-2_f64) * t47216 + F::cast_from(0.68598428988911579156e-3_f64) * t47221 - F::cast_from(0.24009450146119052704e-1_f64) * t47223 - F::cast_from(0.50820002809285328224e-4_f64) * t47227 - F::cast_from(0.34013387707001991332e-1_f64) * t47229 - F::cast_from(0.48018900292238105408e-1_f64) * t47231 - F::cast_from(0.12196800674228478774e-2_f64) * t47235 + F::cast_from(0.30492001685571196935e-3_f64) * t47239 - F::cast_from(0.25724410870841842184e-1_f64) * t3934 * t9955 * t9805 * t9956 + F::cast_from(0.24009450146119052704e0_f64) * t47245 + F::cast_from(0.10289764348336736874e0_f64) * t3934 * t47248 * t3937 * t47249 - F::new(3.0) / F::new(2.0) * t9748 * t800 * t3945 * t3889 - F::cast_from(0.1084295579938911763e-3_f64) * t47259 + F::cast_from(0.13011546959266941156e-2_f64) * t47262 + F::cast_from(0.20579528696673473747e-1_f64) * t13804 * t3936 * t46655 * t47264 - F::cast_from(0.77173232612525526552e-2_f64) * t13804 * t5673 * t9826 * t46416 - F::cast_from(0.30492001685571196936e-2_f64) * t47277;
    t47279
}
