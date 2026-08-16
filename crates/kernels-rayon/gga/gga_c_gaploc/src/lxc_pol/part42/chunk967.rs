//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 967/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk967(t14364: f64, t2089: f64, t12161: f64, t2958: f64, t11807: f64, t12250: f64, t12256: f64, t14377: f64, t14384: f64, t1445: f64, t1991: f64, t2028: f64, t2087: f64, t2103: f64, t3038: f64, t3733: f64, t45251: f64, t45256: f64, t45264: f64, t45269: f64, t45277: f64, t45285: f64, t4673: f64, t47180: f64, t47196: f64, t47199: f64, t47206: f64, t47212: f64, t47215: f64, t50043: f64, t590: f64, t723: f64, t787: f64, t833: f64, t8556: f64) -> (f64, f64) {
    let t50077 = t2089 * t14364;
    let t50092 = t2958 * t12161;
    let t50108 = 0.47667319935800568892e0_f64 * t3733 * t8556 - 0.69017266717057349418e1_f64 * t2087 * t1445 * t50077 * t723 - t45251 + t45256 - t45264 - 0.17875244975925213335e0_f64 * t47180 - 0.79445533226334281487e-1_f64 * t787 * t12250 * t3038 * t2028 - t45269 - 0.21450293971110256002e1_f64 * t12256 * t11807 - t45277 + 0.23005755572352449806e2_f64 * t833 * t1445 * t50043 + 0.23005755572352449806e2_f64 * t833 * t1445 * t50092 + 0.36425779656224712192e1_f64 * t45285 + 0.11916829983950142223e0_f64 * t47196 - 0.59584149919750711116e-1_f64 * t47199 + 0.76685851907841499354e0_f64 * t47206 + 0.95334639871601137787e0_f64 * t2103 * t4673 * t14384 - 0.15337170381568299871e1_f64 * t47212 - 0.76685851907841499354e0_f64 * t47215 + 0.51123901271894332902e0_f64 * t1991 * t14377 * t590;
    (t50092, t50108)
}
