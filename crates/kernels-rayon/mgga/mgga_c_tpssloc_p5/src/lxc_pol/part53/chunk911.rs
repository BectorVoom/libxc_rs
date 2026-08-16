//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 911/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk911(t5161: f64, t8804: f64, t1842: f64, t8800: f64, t3887: f64, t2091: f64, t7936: f64, t12021: f64, t8793: f64, t1807: f64, t8788: f64, t32139: f64, t32141: f64, t32145: f64, t32712: f64, t32715: f64, t32718: f64, t32722: f64, t32724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33793 = t8804 * t5161;
    let t33797 = t8800 * t1842;
    let t33798 = t3887 * t33797;
    let t33804 = t3887 * t2091 * t7936;
    let t33810 = t12021 * t8793 * t1842;
    let t33815 = t1807 * t8788;
    let t33822 = -t32139 - 0.19378922925187387609e-1_f64 * t32712 - t32141 - 0.32298204875312312682e-2_f64 * t32715 + t32718 / 384.0_f64 - t32722 / 384.0_f64 - t32145 - t32724 / 96.0_f64;
    (t33793, t33798, t33804, t33810, t33815, t33822)
}
