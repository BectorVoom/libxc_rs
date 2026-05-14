//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 898/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk898<F: Float>(t12381: F, t72: F, t1355: F, t139: F, t2036: F, t2043: F, t22518: F, t22673: F, t23677: F, t23683: F, t23687: F, t23691: F, t23701: F, t23705: F, t23707: F, t23711: F, t23715: F, t23717: F, t23723: F, t23725: F, t23728: F, t23732: F, t3392: F, t543: F, t5570: F, t5579: F, t5785: F, t5818: F, t8852: F, t8859: F) -> (F, F) {
    let t23733 = t72 * t12381;
    let t23737 = 0.12220869211492952596e0 * t5818 * t23677 - 0.40736230704976508653e-1 * t3392 * t23677 - 0.21895580739717983995e1 * t8859 * t23683 - 0.22653425206514361674e0 * t2043 * t23687 - 0.48327307107230638237e1 * t5785 * t23691 + 0.10947790369858991997e1 * t8852 * t23683 + 0.22653425206514361674e0 * t1355 * t23687 + 0.76518236253115177207e1 * t543 * t22673 - 0.80559205902449556552e-1 * t23701 * t22518 + 0.66678001092592592595e-1 * t23705 * t5570 * t23707 + 0.80559205902449556552e-1 * t23711 * t22518 - 0.66678001092592592595e-1 * t23715 * t5570 * t23717 - 0.14125722719362779757e-1 * t23723 * t23725 + 0.46528109071900715989e1 * t2036 * t23728 * t139 + 0.40006800655555555556e0 * t23732 * t5579 * t23733;
    (t23733, t23737)
}
