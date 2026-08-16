//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1987/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1987<F: Float>(t102487: F, t102488: F, t102490: F, t102492: F, t108554: F, t108559: F, t108562: F, t94444: F, t94460: F, t98141: F, t98148: F, t98161: F) -> F {
    let t109798 = -F::cast_from(0.57165357490759649296e-3_f64) * t108554 + F::cast_from(0.21683201198628406709e-2_f64) * t94444 - F::cast_from(0.60976381323476959249e-3_f64) * t98141 + t102487 + t102488 + F::cast_from(0.43366402397256813419e-2_f64) * t98148 - t102490 - t102492 + F::cast_from(0.2032800112371413129e-4_f64) * t98161 - F::cast_from(0.22866142996303859718e-3_f64) * t108559 - F::cast_from(0.22675591804667994221e-1_f64) * t94460 + F::cast_from(0.10164000561857065645e-3_f64) * t108562;
    t109798
}
