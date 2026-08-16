//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1761/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1761(t136: f64, t4010: f64, t220: f64, t1399: f64, t3945: f64, t9816: f64, t13804: f64, t3889: f64, t3934: f64, t3936: f64, t3937: f64, t46416: f64, t46655: f64, t47216: f64, t47221: f64, t47223: f64, t47227: f64, t47229: f64, t47231: f64, t47235: f64, t47239: f64, t47245: f64, t47248: f64, t47249: f64, t47259: f64, t47262: f64, t47264: f64, t5673: f64, t800: f64, t9748: f64, t9805: f64, t9826: f64, t9955: f64, t9956: f64) -> f64 {
    let t47273 = t4010 * t136;
    let t47274 = t47273 * t220;
    let t47277 = t9816 * t47274 * t3945 * t1399;
    let t47279 = -0.16262400898971305032e-2_f64 * t47216 + 0.68598428988911579156e-3_f64 * t47221 - 0.24009450146119052704e-1_f64 * t47223 - 0.50820002809285328224e-4_f64 * t47227 - 0.34013387707001991332e-1_f64 * t47229 - 0.48018900292238105408e-1_f64 * t47231 - 0.12196800674228478774e-2_f64 * t47235 + 0.30492001685571196935e-3_f64 * t47239 - 0.25724410870841842184e-1_f64 * t3934 * t9955 * t9805 * t9956 + 0.24009450146119052704e0_f64 * t47245 + 0.10289764348336736874e0_f64 * t3934 * t47248 * t3937 * t47249 - 3.0_f64 / 2.0_f64 * t9748 * t800 * t3945 * t3889 - 0.1084295579938911763e-3_f64 * t47259 + 0.13011546959266941156e-2_f64 * t47262 + 0.20579528696673473747e-1_f64 * t13804 * t3936 * t46655 * t47264 - 0.77173232612525526552e-2_f64 * t13804 * t5673 * t9826 * t46416 - 0.30492001685571196936e-2_f64 * t47277;
    t47279
}
