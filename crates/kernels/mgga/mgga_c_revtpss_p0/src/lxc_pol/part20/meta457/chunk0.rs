//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1742/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1742<F: Float>(t1331: F, t9342: F, t9855: F, t2619: F, t9563: F, t3825: F, t9586: F, t1333: F, t14: F, t27: F, t521: F, t583: F, t596: F) -> (F, F, F, F, F, F, F) {
    let t47005 = t9342 * t1331;
    let t47006 = F::new(96.0) * t47005;
    let t47007 = t9855 * t1331;
    let t47008 = F::new(576.0) * t47007;
    let t47009 = t9563 * t2619;
    let t47010 = F::cast_from(0.14649157844805236043e-2_f64) * t47009;
    let t47011 = t3825 * t9586;
    let t47012 = F::cast_from(0.22787578869697033845e-2_f64) * t47011;
    let t47013 = t9342 * t1333;
    let t47014 = F::new(96.0) * t47013;
    let t47016 = t14 * t27 * t521;
    let t47017 = F::new(1440.0) * t47016;
    let t47019 = t583 * t596 * t521;
    (t47006, t47008, t47010, t47012, t47014, t47017, t47019)
}
