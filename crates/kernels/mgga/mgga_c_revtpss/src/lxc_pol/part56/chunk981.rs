//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 981/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk981<F: Float>(t33: F, t32088: F, t33544: F, t57: F, t606: F, t8960: F, t32790: F, t118: F, t1310: F, t2127: F, t2163: F, t32118: F, t32123: F, t32131: F, t32182: F, t32299: F, t32320: F, t32338: F, t32340: F, t32856: F, t32858: F, t32862: F, t32864: F, t32867: F, t33375: F, t508: F, t649: F, t7584: F, t7683: F, t8917: F, t8964: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t33549 = piecewise3::<f64>(t400, t32088, t33544 * t57 / F::new(2.0) - t8960 * t606 / F::new(2.0));
    let t33550 = t32790 + t33549;
    let t33552 = -t118 * t33550 - t1310 * t8917 - F::new(2.0) * t2127 * t7683 - F::new(2.0) * t2163 * t7584 - t33375 * t508 - t649 * t8964 - t32118 - t32123 + t32131 + t32182 + t32299 - t32320 - t32338 - t32340 - F::new(4.0) * t32856 - F::new(4.0) * t32858 - F::new(4.0) * t32862 - F::new(4.0) * t32864 - F::new(4.0) * t32867;
    (t33550, t33552)
}
