//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1334/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1334(t7336: f64, t8775: f64, t10893: f64, t28937: f64, t28941: f64, t28944: f64, t28946: f64, t33820: f64, t33824: f64, t33826: f64, t33829: f64, t33832: f64, t33835: f64, t33838: f64, t33841: f64, t33844: f64, t33846: f64, t5694: f64) -> f64 {
    let t33848 = 0.2780593662921699852e0_f64 * t8775 * t7336;
    let t33849 = -t33820 + 0.92686455430723328401e-1_f64 * t10893 * t5694 + t33824 - t28937 - t28941 + t28944 + t28946 + t33826 - t33829 + t33832 - t33835 - t33838 - t33841 + t33844 + t33846 - t33848;
    t33849
}
