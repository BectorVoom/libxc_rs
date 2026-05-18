//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1127/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1127<F: Float>(t10867: F, t119810: F, t119815: F, t119817: F, t119827: F, t119831: F, t119840: F, t126103: F, t126105: F, t126108: F, t126112: F, t126122: F, t126126: F, t126130: F, t126134: F, t126136: F, t126141: F, t126145: F, t126148: F, t14587: F, t27206: F, t27357: F, t32463: F, t8471: F) -> F {
    let t126153 = F::new(0.28559868832551176308e-1) * t126103 - F::new(0.50779446784275991476e-1) * t126105 + F::new(0.25389723392137995738e-1) * t119810 - t119815 - t119817 + F::new(0.86770434821119025247e-3) * t126108 - F::new(0.33059535666846348619e-4) * t126112 - F::new(0.22847895066040941046e1) * t32463 * t27357 * t27206 + F::new(0.34271842599061411569e1) * t32463 * t10867 * t8471 * t14587 + F::new(0.3718732920905101082e-3) * t126122 - F::new(0.28559868832551176308e-1) * t126126 + F::new(0.42839803248826764462e-1) * t126130 + F::new(0.18822977838986977999e-4) * t126134 - F::new(0.33467254597718846885e-4) * t126136 - F::new(0.14874931683620404328e-2) * t126141 - F::new(0.3718732920905101082e-2) * t126145 + F::new(0.1859366460452550541e-4) * t126148 - F::new(0.76169170176413987214e-1) * t119827 + F::new(0.50779446784275991476e-1) * t119831 - F::new(0.13386901839087538754e-3) * t119840;
    t126153
}
