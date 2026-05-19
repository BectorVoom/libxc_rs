//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 828/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk828<F: Float>(t8772: F, t7616: F, t7617: F, t7622: F, t7625: F, t7629: F, t7645: F, t8219: F, t8220: F, t8221: F, t8776: F, t8780: F, t8784: F, t8788: F, t8794: F) -> F {
    let t9292 = F::new(0.1528125e-1) * t8772;
    let t9298 = t7616 - F::cast_from(0.80031500487063509014e-2_f64) * t7617 + F::cast_from(0.80031500487063509014e-2_f64) * t7622 - t7625 + t7629 + t8219 + t8220 - t8221 - t9292 + F::cast_from(0.10718504529517434243e-2_f64) * t8776 + F::cast_from(0.42874018118069736972e-3_f64) * t8780 - F::cast_from(0.15724046144802076034e-2_f64) * t8784 - F::cast_from(0.94344276868812456207e-3_f64) * t8788 - F::cast_from(0.62896184579208304138e-3_f64) * t8794 + t7645;
    t9298
}
