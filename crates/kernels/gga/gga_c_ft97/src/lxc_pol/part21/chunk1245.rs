//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1245/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1245<F: Float>(t30067: F, t5608: F, t2035: F, t4710: F, t5790: F, t16825: F, t1701: F, t5546: F, t4698: F, t116029: F, t118847: F, t1355: F, t2043: F, t23711: F, t23732: F, t23774: F, t23866: F, t5579: F, t5797: F, t61631: F, t72: F, t76887: F, t76891: F, t8852: F, t8859: F, t94932: F) -> (F, F, F) {
    let t119115 = t30067 * t5608;
    let t119118 = t2035 * t5790 * t4710;
    let t119133 = t1701 * t5546 * t16825;
    let t119137 = t2035 * t5790 * t4698;
    let t119140 = -0.12081826776807659559e1 * t1355 * t118847 - 0.45306850413028723348e0 * t61631 * t5797 - 0.80559205902449556551e-1 * t23711 * t116029 - 0.33339000546296296297e-1 * t119115 - 0.10947790369858991997e1 * t8859 * t119118 + 0.54738951849294959987e0 * t8852 * t119118 + 0.20003400327777777778e0 * t23732 * t5579 * t72 * t76887 - 0.30005100491666666667e0 * t23774 * t5579 * t72 * t76891 + 0.26853068634149852184e-1 * t94932 - 0.22653425206514361674e0 * t2043 * t119133 + 0.21895580739717983994e1 * t23866 * t119137;
    (t119133, t119137, t119140)
}
