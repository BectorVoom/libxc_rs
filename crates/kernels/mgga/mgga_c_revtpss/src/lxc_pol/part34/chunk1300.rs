//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1300/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1300<F: Float>(t100567: F, t1089: F, t1646: F, t1668: F, t1982: F, t1986: F, t23583: F, t23617: F, t24042: F, t24068: F, t25605: F, t25611: F, t25651: F, t25671: F, t25699: F, t27419: F, t27550: F, t27661: F, t27699: F, t29731: F, t29818: F, t29844: F, t29866: F, t29883: F, t3304: F, t6244: F, t6258: F, t6259: F, t6299: F, t6305: F, t6393: F, t7102: F, t7144: F, t7145: F, t7160: F, t7810: F, t7821: F, t94026: F, t99934: F) -> F {
    let t114009 = -F::new(0.4336814094102599731e0) * t1982 * t24042 * t1986 - F::new(0.10408353825846239354e2) * t27419 * t29818 - F::new(0.78062653693846795158e1) * t25699 * t7145 * t7810 * t6244 - F::new(0.39512695097613069591e1) * t25651 * t23583 + F::new(0.10408353825846239354e2) * t99934 * t29844 - F::new(0.19756347548806534796e1) * t27699 * t6393 - F::new(0.39512695097613069591e1) * t94026 * t24068 - F::new(0.26020884564615598386e1) * t25671 * t100567 * t6305 * t3304 - F::new(0.78062653693846795158e1) * t25699 * t7145 * t7821 * t6258 + F::new(0.52041769129231196772e1) * t7144 * t7160 * t29731 * t1646 + F::new(0.26020884564615598386e1) * t25611 * t29883 * t1668 * t1089 + F::new(0.26020884564615598386e1) * t25611 * t7821 * t6299 * t1089 + F::new(0.10408353825846239354e2) * t27661 * t29866 + F::new(0.19756347548806534796e1) * t7102 * t23617 - F::new(0.19756347548806534796e1) * t27550 * t6259 + F::new(0.26020884564615598386e1) * t25605 * t29731 * t1668 * t1089;
    t114009
}
