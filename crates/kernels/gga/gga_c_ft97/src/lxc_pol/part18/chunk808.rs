//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 808/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk808<F: Float>(t1294: F, t22679: F, t1685: F, t1701: F, t5546: F, t39: F, t78: F, t388: F, t428: F, t5790: F, t5517: F, t66: F, t1295: F, t1300: F, t1683: F, t1698: F, t2035: F, t22619: F, t22620: F, t22623: F, t22629: F, t22634: F, t22639: F, t22644: F, t22652: F, t22657: F, t22661: F, t22667: F, t22677: F, t399: F, t401: F, t5518: F, t5523: F, t5534: F, t5545: F, t5587: F, t7867: F, t7889: F) -> (F, F) {
    let t22680 = t22679 * t1294;
    let t22683 = t1701 * t5546 * t1685;
    let t22686 = t78 * t39;
    let t22687 = t388 * t22686;
    let t22692 = t5790 * t428;
    let t22696 = t5517 * t66;
    let t22699 = -0.89080607335887169332e-3 * t22619 * t22620 - 0.10417318313778431088e-5 * t22623 * t22629 + 0.25537443351851851852e-1 * t22634 + 0.5297955163169938709e-2 * t5587 * t22639 - 0.30274029503828221194e-3 * t22644 + 0.2370952259137005195e-1 * t5534 * t399 + 0.28107073075534343171e-3 * t1295 * t1698 + 0.2370952259137005195e-1 * t5518 * t399 + 0.47419045182740103901e-1 * t5545 * t1701 * t22652 * t401 - 0.44455354858818847408e-2 * t7889 * t1701 * t22657 - 0.2370952259137005195e-1 * t1300 * t1701 * t22661 - 0.75080154872671831175e-1 * t1295 * t1683 + 4.0 * t22667 + 2.0 * t22677 + 2.0 * t22680 - 0.44455354858818847408e-2 * t5545 * t22683 - 0.2108030480665075738e-3 * t22687 * t2035 * t5790 * t401 + 0.1054015240332537869e-3 * t7867 * t2035 * t22692 - 4.0 * t22696 * t5523;
    (t22696, t22699)
}
