//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 967/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk967<F: Float>(t14364: F, t2089: F, t12161: F, t2958: F, t11807: F, t12250: F, t12256: F, t14377: F, t14384: F, t1445: F, t1991: F, t2028: F, t2087: F, t2103: F, t3038: F, t3733: F, t45251: F, t45256: F, t45264: F, t45269: F, t45277: F, t45285: F, t4673: F, t47180: F, t47196: F, t47199: F, t47206: F, t47212: F, t47215: F, t50043: F, t590: F, t723: F, t787: F, t833: F, t8556: F) -> (F, F) {
    let t50077 = t2089 * t14364;
    let t50092 = t2958 * t12161;
    let t50108 = F::new(0.47667319935800568892e0) * t3733 * t8556 - F::new(0.69017266717057349418e1) * t2087 * t1445 * t50077 * t723 - t45251 + t45256 - t45264 - F::new(0.17875244975925213335e0) * t47180 - F::new(0.79445533226334281487e-1) * t787 * t12250 * t3038 * t2028 - t45269 - F::new(0.21450293971110256002e1) * t12256 * t11807 - t45277 + F::new(0.23005755572352449806e2) * t833 * t1445 * t50043 + F::new(0.23005755572352449806e2) * t833 * t1445 * t50092 + F::new(0.36425779656224712192e1) * t45285 + F::new(0.11916829983950142223e0) * t47196 - F::new(0.59584149919750711116e-1) * t47199 + F::new(0.76685851907841499354e0) * t47206 + F::new(0.95334639871601137787e0) * t2103 * t4673 * t14384 - F::new(0.15337170381568299871e1) * t47212 - F::new(0.76685851907841499354e0) * t47215 + F::new(0.51123901271894332902e0) * t1991 * t14377 * t590;
    (t50092, t50108)
}
