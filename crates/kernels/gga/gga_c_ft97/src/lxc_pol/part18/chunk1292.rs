//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1292/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1292<F: Float>(t100634: F, t101089: F, t104797: F, t104813: F, t104819: F, t104824: F, t104834: F, t104838: F, t18: F, t2043: F, t23715: F, t23732: F, t23832: F, t23839: F, t23878: F, t26635: F, t26692: F, t3384: F, t423: F, t538: F, t5579: F, t5785: F, t5818: F, t61641: F, t6597: F, t72: F, t8859: F, t935: F, t94443: F, t94447: F, t94697: F, t94891: F, t94892: F) -> (F,) {
    let t104841 = -0.55565000910493827163e-2 * t94443 - 0.74086667880658436217e-2 * t94447 - 0.12220869211492952596e0 * t5818 * t104797 + 0.45306850413028723348e0 * t23878 * t6597 + 0.10947790369858991998e1 * t94891 * t94892 * t3384 - 0.88904001456790123461e-1 * t26692 * t101089 - 0.13335600218518518519e0 * t23715 * t100634 * t423 * t18 * t538 - 0.22653425206514361674e0 * t2043 * t104813 - 0.4708574239787593252e-2 * t94697 * t935 + 0.48335523541469733928e0 * t104819 * t26635 + 0.48335523541469733928e0 * t23832 * t104824 - 0.48335523541469733928e0 * t23839 * t104824 + 0.40006800655555555556e0 * t23732 * t5579 * t72 * t61641 - 0.48327307107230638238e1 * t5785 * t104834 + 0.18611243628760286395e2 * t8859 * t104838;
    (t104841,)
}
