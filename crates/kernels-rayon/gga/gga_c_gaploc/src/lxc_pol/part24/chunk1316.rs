//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1316/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1316(t33580: f64, t10984: f64, t11033: f64, t1445: f64, t1457: f64, t2004: f64, t2005: f64, t2178: f64, t28715: f64, t32180: f64, t32230: f64, t33544: f64, t33546: f64, t33560: f64, t33564: f64, t33567: f64, t33569: f64, t33572: f64, t33574: f64, t33576: f64, t5703: f64, t833: f64) -> f64 {
    let t33581 = 0.85206502119823888168e-1_f64 * t33580;
    let t33582 = -t33544 - t33546 + t28715 + 0.43710935587469654631e2_f64 * t833 * t1445 * t32230 + 0.46011511144704899612e1_f64 * t2178 * t11033 + 0.71500979903700853338e0_f64 * t2004 * t1457 * t32180 + 0.71500979903700853338e0_f64 * t5703 * t10984 - t33560 + t33564 + t33567 + t33569 + t33572 + t33574 + 0.21450293971110256002e1_f64 * t33576 * t2005 + t33581;
    t33582
}
