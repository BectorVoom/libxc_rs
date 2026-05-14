//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1358/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1358<F: Float>(t33597: F, t3739: F, t33610: F, t33509: F, t110384: F, t110423: F, t110474: F, t110635: F, t113629: F, t113673: F, t113765: F, t113815: F, t113821: F, t113832: F, t113846: F, t113851: F, t18953: F, t19033: F, t32008: F, t32087: F, t32102: F, t33408: F, t33410: F, t6175: F, t6183: F) -> (F, F, F, F) {
    let t113853 = t3739 * t33597;
    let t113854 = 0.22109259259259259258e-2 * t113853;
    let t113855 = t3739 * t33610;
    let t113856 = 0.66327777777777777776e-2 * t113855;
    let t113857 = t3739 * t33509;
    let t113859 = 0.34722222222222222223e-2 * t32087 * t113815 - 0.40208333333333333334e-2 * t32008 * t113821 + 0.13888888888888888889e-1 * t110423 * t33410 + 0.13888888888888888889e-1 * t110384 * t33410 + 0.69444444444444444446e-2 * t32087 * t6183 * t33408 * t18953 + 0.27777777777777777778e-1 * t32087 * t6175 * t113832 * t19033 - 0.23280625000000000001e-2 * t32102 * t113765 + 0.17972642500000000001e-2 * t110474 * t113629 - 0.15520416666666666667e-2 * t110635 * t113673 - 0.80416666666666666667e-2 * t32008 * t113673 - 0.69444444444444444446e-2 * t32087 * t113846 + 0.99491666666666666664e-2 * t113851 - t113854 + t113856 - 0.22109259259259259258e-2 * t113857;
    (t113853, t113855, t113857, t113859)
}
