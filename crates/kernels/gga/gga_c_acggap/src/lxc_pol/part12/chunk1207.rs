//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1207/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1207<F: Float>(t35882: F, t35885: F, t31607: F, t31609: F, t31612: F, t31619: F, t31623: F, t31625: F, t31627: F, t35860: F, t35864: F, t35866: F, t35868: F, t35872: F, t35874: F, t35876: F, t35879: F, t35887: F) -> F {
    let t37757 = t35882 / F::new(64.0);
    let t37758 = t35885 / F::new(192.0);
    let t37766 = F::new(0.305625e-1) * t35860 - F::cast_from(0.15724046144802076034e-2_f64) * t35864 + F::cast_from(0.13719685797782315831e-1_f64) * t35866 - F::cast_from(0.13719685797782315831e-1_f64) * t35868 + F::cast_from(0.18868855373762491241e-2_f64) * t35872 - F::cast_from(0.51448821741683684367e-2_f64) * t35874 + F::cast_from(0.75475421495049964965e-2_f64) * t35876 - F::new(11.0) / F::new(96.0) * t31607 - t35879 / F::new(48.0) - t37757 - t37758 - t35887 / F::new(12.0) - F::cast_from(0.38586616306262763276e-1_f64) * t31609 + F::cast_from(0.34299214494455789577e-2_f64) * t31612 + F::cast_from(0.37737710747524982482e-1_f64) * t31619 - F::cast_from(0.42874018118069736972e-3_f64) * t31623 + F::cast_from(0.51448821741683684368e-2_f64) * t31625 + F::cast_from(0.10289764348336736874e-1_f64) * t31627;
    t37766
}
