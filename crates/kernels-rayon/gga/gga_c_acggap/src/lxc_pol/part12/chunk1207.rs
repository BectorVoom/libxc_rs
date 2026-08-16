//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1207/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1207(t35882: f64, t35885: f64, t31607: f64, t31609: f64, t31612: f64, t31619: f64, t31623: f64, t31625: f64, t31627: f64, t35860: f64, t35864: f64, t35866: f64, t35868: f64, t35872: f64, t35874: f64, t35876: f64, t35879: f64, t35887: f64) -> f64 {
    let t37757 = t35882 / 64.0_f64;
    let t37758 = t35885 / 192.0_f64;
    let t37766 = 0.305625e-1_f64 * t35860 - 0.15724046144802076034e-2_f64 * t35864 + 0.13719685797782315831e-1_f64 * t35866 - 0.13719685797782315831e-1_f64 * t35868 + 0.18868855373762491241e-2_f64 * t35872 - 0.51448821741683684367e-2_f64 * t35874 + 0.75475421495049964965e-2_f64 * t35876 - 11.0_f64 / 96.0_f64 * t31607 - t35879 / 48.0_f64 - t37757 - t37758 - t35887 / 12.0_f64 - 0.38586616306262763276e-1_f64 * t31609 + 0.34299214494455789577e-2_f64 * t31612 + 0.37737710747524982482e-1_f64 * t31619 - 0.42874018118069736972e-3_f64 * t31623 + 0.51448821741683684368e-2_f64 * t31625 + 0.10289764348336736874e-1_f64 * t31627;
    t37766
}
