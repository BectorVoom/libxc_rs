//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 953/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk953<F: Float>(t112778: F, t112803: F, t112818: F, t112820: F, t112773: F, t112782: F, t112784: F, t112788: F, t112795: F, t112798: F, t112807: F, t112811: F, t112814: F) -> F {
    let t114714 = F::cast_from(0.5383034145885385447e-3_f64) * t112778;
    let t114720 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t112803;
    let t114724 = F::cast_from(0.32298204875312312682e-2_f64) * t112818;
    let t114725 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t112820;
    let t114726 = t112773 / F::cast_from(96.0_f64) + t114714 + F::cast_from(0.67826230238155856632e-1_f64) * t112782 + F::cast_from(0.13565246047631171327e0_f64) * t112784 - F::cast_from(0.96894614625936938046e-2_f64) * t112788 + t112795 / F::cast_from(384.0_f64) - t112798 / F::cast_from(384.0_f64) + t114720 - t112807 / F::cast_from(768.0_f64) - t112811 / F::cast_from(768.0_f64) + F::cast_from(0.32298204875312312682e-2_f64) * t112814 + t114724 + t114725;
    t114726
}
