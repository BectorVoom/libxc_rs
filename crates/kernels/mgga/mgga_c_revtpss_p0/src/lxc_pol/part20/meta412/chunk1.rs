//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1523/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1523<F: Float>(t11255: F, t42668: F, t1068: F, t11259: F, t11875: F, t247: F, t3116: F, t3117: F, t3162: F, t42883: F, t42886: F, t42889: F, t42892: F, t42894: F, t42900: F, t42902: F, t42904: F, t42907: F, t42909: F, t4837: F) -> F {
    let t42914 = t42668 * t11255;
    let t42917 = F::cast_from(0.57927562257303111285e-1_f64) * t42883 - F::cast_from(0.22866142996303859719e-2_f64) * t42886 + F::cast_from(0.19055119163586549765e-2_f64) * t42889 - F::cast_from(0.19055119163586549765e-2_f64) * t42892 + F::cast_from(0.12862205435420921092e-2_f64) * t11875 * t3117 * t42894 * t3162 + F::cast_from(0.17149607247227894789e-2_f64) * t42900 + F::cast_from(0.11433071498151929859e-2_f64) * t42902 + F::cast_from(0.57165357490759649296e-3_f64) * t42904 * t1068 - F::cast_from(0.3811023832717309953e-3_f64) * t42907 + F::cast_from(0.17149607247227894789e-2_f64) * t4837 * t247 * t3116 * t42909 + F::cast_from(0.85748036236139473944e-3_f64) * t42914 * t11259;
    t42917
}
