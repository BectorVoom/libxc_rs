//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1004/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1004<F: Float>(t12920: F, t5268: F, t1042: F, t11231: F, t1261: F, t12847: F, t12853: F, t12855: F, t12858: F, t12862: F, t12866: F, t12868: F, t12872: F, t12876: F, t12882: F, t12887: F, t12890: F, t12893: F, t12895: F, t12900: F, t12902: F, t12905: F, t12907: F, t12910: F, t12912: F, t12918: F, t3711: F, t3718: F, t484: F, t5331: F, t5340: F) -> F {
    let t12921 = t5268 * t12920;
    let t12922 = t1042 * t12921;
    let t12925 = t5268 * t11231;
    let t12926 = t1042 * t12925;
    let t12929 = F::new(0.42874018118069736972e-3) * t5331 * t12847 + t12853 - F::new(0.12862205435420921092e-2) * t12855 * t12858 - F::new(0.64311027177104605458e-3) * t3718 * t12862 + F::new(0.85748036236139473944e-3) * t12866 * t12868 + F::new(0.12862205435420921092e-2) * t5340 * t12872 - F::new(0.64311027177104605458e-3) * t5331 * t12876 + F::new(0.95275595817932748825e-4) * t12882 + F::new(0.47637797908966374413e-3) * t12887 + F::new(0.21437009059034868486e-3) * t12890 * t484 - F::new(0.14291339372689912324e-3) * t12893 + F::new(0.42874018118069736972e-3) * t12895 + t12900 + F::new(0.85748036236139473944e-3) * t12902 - F::new(0.14291339372689912324e-3) * t12905 + F::new(0.85748036236139473944e-3) * t12907 + F::new(0.12862205435420921092e-2) * t12910 * t12912 - F::new(0.85748036236139473944e-3) * t12918 + F::new(0.85748036236139473944e-3) * t3711 * t12922 - F::new(0.85748036236139473944e-3) * t1261 * t12926;
    t12929
}
